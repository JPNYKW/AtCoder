// https://atcoder.jp/contests/abc150/submissions/10774151

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#define yes "Yes\n"
#define no "No\n"

int n, c;
char s[51];

int main(void){
  scanf("%d %s", &n,s);

  for (int i = 0; i < n - 2; i++) {
    char buf[4];
    strncpy(buf, s + i, 3);
    if (strcmp(buf, "ABC") == 0) c++;
  }

  printf("%d\n", c);
  return 0;
}
