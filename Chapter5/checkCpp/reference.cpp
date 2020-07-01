#include <iostream>

int main() {
  int x = 10;
  int &r = x;
  if(r == 10) {
    std::cout << "Ok" << std::endl;
  }
  r = 20;
}
