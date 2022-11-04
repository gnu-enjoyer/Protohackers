#include "server.h"

int main() {

  auto socket = std::make_unique<Socket>(45100);

  if (socket)
    socket->Poll();
}
