#pragma once

#include <memory>

class Socket {
  int fd = -1;

public:
  [[noreturn]] void Poll();
  explicit Socket(const int& port);
  ~Socket();
};
