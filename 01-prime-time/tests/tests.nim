import std/[asyncdispatch, json, options]
import ../src/main

proc asyncTests() {.async.} =
  block:
    let res = await isPrime(89)
    assert res
  block:
    let jsonNode = parseJson("""{"method":"isPrime","number":123}""")
    let json = await parsePackets($jsonNode)
    assert json.isSome
  block:
    let jsonNode = parseJson("""{"methd":"isPrme","nuber":12}""")
    let json = await parsePackets($jsonNode)
    assert json.isNone

waitFor asyncTests()
