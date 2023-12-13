
// generate given amount of fake data to be hashed, and push it to standard output

#include <stdio.h>
#include <stdlib.h>

void generate(int megabytes) {
  
  unsigned char buffer[257];
  for(int i=0;i<256;i++) { buffer[i] = 1+(i%255); }
  buffer[256] = 0;

  for(int i=0;i<megabytes;i++) {
    for(int j=0;j<4096;j++) {
      fputs( (char*)buffer, stdout);
    }
  }

}

int main( int argc, char *argv[] ) {
  
  int megabytes = 1;
  switch(argc) {

    case 2:
      megabytes = atoi(argv[1]);
      generate(megabytes);
      break;

    default:
      printf("usage:\n");
      printf("$ fakedata <amount_in_megabytes>:\n");
      exit(-1);
      break;

  }
}