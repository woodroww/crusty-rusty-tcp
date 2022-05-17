# crusty-rusty-tcp

So I'm following along with Jon Gjengset's YouTube video series on tcp in rust.
[YouTube](https://youtube.com/playlist?list=PLqbS7AVVErFivDY3iKAQk3_VAm8SXwt1X) 
[Jon's GitHub](https://github.com/jonhoo/rust-tcp)

RFC 793 (Transmission Control Protocol)
https://datatracker.ietf.org/doc/html/rfc793

RFC 2525 (Known TCP Implementation Problems)
https://datatracker.ietf.org/doc/rfc2525/ 

IP protocol Numbers
https://www.iana.org/assignments/protocol-numbers/protocol-numbers.txt

## netcat
Send packets through to the TUN, from address 192.168.0.2 on port 443
```
nc 192.168.0.2 443
```
