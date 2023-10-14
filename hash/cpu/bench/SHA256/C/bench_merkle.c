// linear hashing 2048 byte chunks then building a Merkle tree on the top of them
// using `nThreads` parallel threads

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <pthread.h>

#include "sha2.h"

/*

void SHA256_Init(SHA256_CTX *);
void SHA256_Update(SHA256_CTX*, const uint8_t*, size_t);
void SHA256_Final(uint8_t[SHA256_DIGEST_LENGTH], SHA256_CTX*);

*/

//------------------------------------------------------------------------------

void print_hash(uint8_t *hash) {
  for(int i=0;i<SHA256_DIGEST_LENGTH;i++) {
    printf("%02x",hash[i]);
  } 
  printf("\n");
}

int integerLog2(int a) {
  int log = -1;
  while(a>0) { log++; a=a>>1; }
  return log;
}

//------------------------------------------------------------------------------

#define CHUNK_SIZE 2048
#define CHUNKS_PER_MEGABYTE ((1024*1024)/(CHUNK_SIZE))

void hash_chunk(int idx, uint8_t *chunk_data, uint8_t *tgt) {
  SHA256_CTX ctx;
  SHA256_Init(&ctx);
  SHA256_Update(&ctx, chunk_data, CHUNK_SIZE );
  SHA256_Final( tgt, &ctx );
}

//------------------------------------------------------------------------------

void sha256_compression( const uint8_t *src, uint8_t *tgt ) {
  SHA256_CTX ctx;
  SHA256_Init(&ctx);
  SHA256_Update(&ctx, src, 2*SHA256_DIGEST_LENGTH );
  SHA256_Final( tgt, &ctx );
}

void calc_merkle_root( int logN, uint8_t *data, uint8_t *tgt ) {

  int N = (1<<logN);
  
  uint8_t *layers = malloc( (N-1)*SHA256_DIGEST_LENGTH );
  assert( layers != 0 );

  const uint8_t *p = data;
  uint8_t       *q = layers;
  for(int k=0; k<logN; k++) {
    int M = (1<<(logN-1-k));
    for(int j=0; j<M; j++) {
      sha256_compression( p + 2*j*SHA256_DIGEST_LENGTH , q + j*SHA256_DIGEST_LENGTH );
    }
    p =  q;
    q += M*SHA256_DIGEST_LENGTH;
  }

  memcpy( tgt, p, SHA256_DIGEST_LENGTH );

  free(layers);

}

//------------------------------------------------------------------------------

typedef struct {
  int      tidx;
  int      linear_idx;
  int      nChunks;
  uint8_t  *tgt;
} ThreadData;

void *my_thread_fun(void *ptr) { 
  ThreadData *thread_data = (ThreadData*)ptr;
  uint8_t *leaves = malloc( SHA256_DIGEST_LENGTH * thread_data->nChunks );

  // cretae fake data
  unsigned char buffer[CHUNK_SIZE];
  for(int i=0;i<CHUNK_SIZE;i++) { 
    // to be compatible with the unix fakedata thingy
    buffer[i] = 1+((i%256)%255); 
  } 

  // calculate the leaf hashes
  for(int i=0; i<thread_data->nChunks; i++) {
    hash_chunk(i, buffer, leaves + i*SHA256_DIGEST_LENGTH );
  }

  // calculate the merkle root of the subtree
  int logNChunks = integerLog2(thread_data->nChunks);
  calc_merkle_root( logNChunks , leaves, thread_data->tgt );

  free(leaves);
  pthread_exit(NULL);
} 

// -----------------------------------------------------------------------------

// we assume nThreads is a power of two
// and that the number of chunks per thread is also a power of two
void merkle_root_multithread(int megabytes, int nThreads) {

  int logNThreads = integerLog2(nThreads);
  printf("nThreads = %d; logNThreads = %d\n",nThreads,logNThreads);
  assert( nThreads == (1<<logNThreads) );

  int64_t chunks_per_thread = (1024*1024*megabytes) / nThreads / CHUNK_SIZE;
  int logNChunks = integerLog2(chunks_per_thread);
  printf("chunks per thread = %lld | log of that = %d\n", chunks_per_thread, logNChunks );
  assert( chunks_per_thread == (1<<logNChunks) );


  // create the worker threads
  uint8_t    *subtree_roots = malloc( SHA256_DIGEST_LENGTH *nThreads );
  pthread_t  *thread_ids    = malloc( sizeof(pthread_t )   *nThreads );
  ThreadData *thread_data   = malloc( sizeof(ThreadData)   *nThreads );
  for(int t=0; t<nThreads; t++) {
    thread_data[t].tidx       = t;
    thread_data[t].nChunks    = chunks_per_thread;
    thread_data[t].tgt        = subtree_roots + t*SHA256_DIGEST_LENGTH;
    thread_data[t].linear_idx = t * chunks_per_thread;
    pthread_create(thread_ids+t, NULL, my_thread_fun, thread_data+t); 
  }

  // wait for the threads to finish
  for(int t=0; t<nThreads; t++) {
    pthread_join( thread_ids[t], NULL );
  }

  // calculate the final root
  uint8_t root[SHA256_DIGEST_LENGTH];
  calc_merkle_root( logNThreads, subtree_roots, root );

  printf("\nMerkle root of the linear SHA256 hashes of the chunks:\n");
  print_hash(root);

  free(thread_data);
  free(subtree_roots);
}

// -----------------------------------------------------------------------------

int main( int argc, char *argv[] ) {
  
  int nThreads  = 1;
  int megabytes = 1;

  switch(argc) {

    case 2:
      megabytes = atoi(argv[1]);
      break;

    case 3:
      megabytes = atoi(argv[1]);
      nThreads  = atoi(argv[2]);
      break;

    default:
      printf("usage:\n");
      printf("$ ./bench_merkle <megabytes>:\n");
      printf("$ ./bench_merkle <megabytes> <nthreads>:\n");
      exit(-1);
      break;
  }

  printf("calculating SHA2 chunked Merkle root for %d Mb of data on %d threads...\n",megabytes,nThreads);
  merkle_root_multithread(megabytes, nThreads);    
  
  printf("\n");
}
