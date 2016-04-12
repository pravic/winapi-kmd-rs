## 01.minimal

Minimal Windows kernel driver written in Rust. [DriverEntry](https://msdn.microsoft.com/en-us/library/windows/hardware/ff544113%28v=vs.85%29.aspx) just prints `hello` and quits immediately.


## 02.unload

Simple kernel driver which supports [unloading](https://msdn.microsoft.com/en-us/library/windows/hardware/ff564886%28v=vs.85%29.aspx).


## 03.urandom

[devrandom](https://github.com/pravic/ontl/tree/master/samples/devrandom) driver sample which has been ported to Rust.

It creates `\\.\urandom` device, which can produce random data like `/dev/urandom`, but insecure.

This sample shows how to create a [Device Object](https://msdn.microsoft.com/en-us/library/windows/hardware/ff548014%28v=vs.85%29.aspx), assotiate it with user-mode visible [name](https://msdn.microsoft.com/en-us/library/windows/hardware/ff556420%28v=vs.85%29.aspx) and process [I/O requests](https://msdn.microsoft.com/en-us/library/windows/hardware/ff544248%28v=vs.85%29.aspx) from user-mode applications.


### Screenshots

[![Registered device](http://savepic.su/7182468m.png)](http://savepic.su/7182468.png)

[![Communicating](http://savepic.su/7183492m.png)](http://savepic.su/7183492.png)
