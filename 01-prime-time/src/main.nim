
import std/[asyncdispatch, asyncnet, json, options]

type
  Request = object
    `method`: string
    number: int

proc isPrime*(n: int): Future[bool] {.async.} =
  if n > 1:
    for i in countdown(n-1, 2):
      if n mod i == 0:
        return false
    return true
  else:
    return false

proc parsePackets*(s: string): Future[Option[Request]] {.async.} =
  try:
    let json = parseJson(s)
    return some(to(json, Request))
  except:
    return

proc processClient(client: AsyncSocket) {.async.} =
  while true:
    var msg = client.recvLine()

    if await withTimeout(msg, 1000):
      let input = await msg

      if input != "":
        let request = await parsePackets(input)

        if request.isSome:
          let response = %* {"method": "isPrime", "prime": await isPrime(
              request.get.number)}
          await client.send($response & "\n")
        else:
          await client.send("malformed\n")
          client.close()
          break

      else:
        client.close()
        break

proc serve() {.async.} =
  var server = newAsyncSocket()
  server.setSockOpt(OptReuseAddr, true)
  server.bindAddr(Port(45100))
  server.listen()

  while true:
    let client = await server.accept()
    asyncCheck processClient(client)

when isMainModule:
  asyncCheck serve()
  runForever()
