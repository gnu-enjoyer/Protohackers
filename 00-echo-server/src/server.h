#pragma once

#include <memory>
#include <netinet/in.h>

class Socket {
  int fd = -1;
  sockaddr_in6 serverAddr{.sin6_family = AF_INET6, .sin6_addr = in6addr_any};

public:
  [[noreturn]] void Poll();
  explicit Socket(const int& port);
  ~Socket();
};
