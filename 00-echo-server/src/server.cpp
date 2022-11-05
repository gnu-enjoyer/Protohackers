#include "server.h"

#include <array>
#include <netinet/in.h>
#include <stdexcept>
#include <sys/socket.h>
#include <thread>
#include <unistd.h>

void connectionHandler(const int& fd) {
  std::array<char, 4096> buffer{};

  while (true) {
    buffer = {0};

    auto incoming = recv(fd, buffer.data(), buffer.size(), 0);

    if (incoming > 0)
      send(fd, buffer.data(), incoming, 0);
    else
      break;
  }

  close(fd);
}

void Socket::Poll() {
  while (true) {

    sockaddr_in client{};
    socklen_t clientSize = sizeof(client);

    int remote = accept(fd, (sockaddr*)&client, &clientSize);

    if (remote != -1)
      std::thread(connectionHandler, remote).detach();
  }
}

Socket::Socket(const int& port) {
  const int reuse = 1;
  sockaddr_in6 serverAddr{.sin6_family = AF_INET6,
                          .sin6_port = htons(port),
                          .sin6_addr = in6addr_any};
  try {
    fd = socket(AF_INET6, SOCK_STREAM, 0);

    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &reuse, sizeof(reuse));

    bind(fd, (sockaddr*)&serverAddr, sizeof(serverAddr));

    listen(fd, SOMAXCONN);
  } catch (...) {
    throw std::runtime_error("Error initialising socket.");
  }
}

Socket::~Socket() { close(fd); }
