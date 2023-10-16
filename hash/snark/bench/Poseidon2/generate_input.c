
#include <stdio.h>
#include <stdlib.h>

void generate(int n) {

  printf("{ \"inp\":\n" );
  for(int i=0;i<n;i++) {
    char c = (i==0)?'[':',';
    printf("%c%d",c,i+1);
    if ((i%10)==9) { printf("\n"); }
  }
  printf("]\n");
  printf("}\n");

}

int main( int argc, char *argv[] ) {
  
  int nElements = 1;
  switch(argc) {

    case 2:
      nElements = atoi(argv[1]);
      generate(nElements);
      break;

    default:
      printf("usage:\n");
      printf("$ ./generate_input <number_of_field_elements>:\n");
      exit(-1);
      break;

  }
}

