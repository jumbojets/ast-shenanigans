//11
//if (if (false) then { false } else { true }) then { (10) + (1) } else { -1 }
#include <stdbool.h>
#include <stdio.h>
int main() {
bool y = false;
int e;
if (y) {
bool ia = false;
e = ia;
} else {
bool cb = true;
e = cb;
}
int a;
if (e) {
int n = 10;
int u = 1;
int g = n + u;
a = g;
} else {
int k = -1;
a = k;
}
printf("%d\n", a);
return 0;
}
