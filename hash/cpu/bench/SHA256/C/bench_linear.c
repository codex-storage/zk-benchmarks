// linear hashing of fake data 

#include <stdio.h>
#include <stdlib.h>
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

//------------------------------------------------------------------------------

#define BUF_SIZE 2048

void generate_and_hash(int megabytes) {

  SHA256_CTX ctx;
  SHA256_Init(&ctx);

  unsigned char buffer[BUF_SIZE];
  for(int i=0;i<BUF_SIZE;i++) { 
    // to be compatible with the unix fakedata thingy
    buffer[i] = 1+((i%256)%255); 
  } 

  int chunks_per_megabyte = (1024*1024)/BUF_SIZE;
  for(int i=0;i<megabytes;i++) {
    for(int j=0;j<chunks_per_megabyte;j++) {
      SHA256_Update(&ctx, buffer, BUF_SIZE );
    }
  }

  uint8_t hash[SHA256_DIGEST_LENGTH]; 
  SHA256_Final( hash, &ctx );
  print_hash(hash);

}

//------------------------------------------------------------------------------

int main( int argc, char *argv[] ) {
  
  int megabytes = 1;
  switch(argc) {

    case 2:
      megabytes = atoi(argv[1]);
      printf("SHA256 hashing %d megabytes of fake data\n",megabytes);
      generate_and_hash(megabytes);
      break;

    default:
      printf("usage:\n");
      printf("$ bench_linear <amount_in_megabytes>:\n");
      exit(-1);
      break;

  }
}