pub fn enum_definition() {
    {
        enum IpAddrKind {
            V4,
            V6,
        }

        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let four = IpAddrKind::V4;
        let v6 = IpAddrKind::V6;

        route(IpAddrKind::V4);
        route(IpAddrKind::V6);

        fn route(ip_kind: IpAddrKind) {}

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    {
        enum IpAddr2 {
            V4(String),
            V6(String),
        }

        let home = IpAddr2::V4(String::from("127.0.0.1"));
        let loopback = IpAddr2::V6(String::from("::1"));
    }

    {
        enum IpAddr3 {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr3::V4(127, 0, 0, 1);
        let loopback = IpAddr3::V6(String::from("::1"));
    }

    {
        // standard library
        struct Ipv4Addr {
            // --snip--
        }

        struct Ipv6Addr {
            // --snip--
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }

    {}
}

pub fn enum_and_struct() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 在这里定义方法体
            println!("Message: {:#?}", self);
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    struct QuitMessage; // 类单元结构体
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // 元组结构体
    struct ChangeColorMessage(i32, i32, i32); // 元组结构体
}

pub fn enum_option() {
    let some_number = Some(0);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_number: {:?}", some_char);
    println!("some_number: {:?}", absent_number);
}
