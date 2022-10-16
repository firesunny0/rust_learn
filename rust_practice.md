#### Basic Types 
1. let v: u16 = 38_u8 as u16;
2. format!("{}", std::any::type_name::<T>())
3. assert!(0.1+0.2==0.3); // f32  or  abs
4. for i in -3..2 {}
5. assert_eq!((1..5), Range{ start: 1, end: 5 });
6. assert_eq!((1..=5), RangeInclusive::new(1, 5));
7. size_of_val(&c); // unicode codec  4
8. assert!(size_of_val(&unit) == 0); // unit = ();
9. thread::sleep(time::Duration::from_secs(1));
10. unimplemented!()
11. panic!()
12. todo!()
    
#### Ownership
1. &mut var ,  &var
2. let Person { name, ref age } = person;

#### Compound-types
1. Box<str>  --   Box<&str>
2. s.push(',')  -- s.push_str(" xxx")
3. s += &"!".to_string()  
4. let s1 = s.replace("dogs", "cats");
5. for c in "你好，世界".chars()
6. let h = &s1[0..1];
7. let quotes = r#"And then I said: "There is no escape!""#;
8. let  delimiter = r###"A string with "# in it. And even "##!"###;
9. “###  怎么办
10. let rocket = utf8_slice::slice(s, 4, 5)
11. let list: [i32; 100] = [1;100]
12. assert!(std::mem::size_of_val(&slice) == 16);
13. char unicode 4  string utf8 1英 2中
14. &String &str
15. let slice = &s[0..3];    assert!(slice == "你");
16. println!("too long tuple: {:?}", too_long_tuple);
17. 对于结构体，我们必须为其中的每一个字段都指定具体的值
18. 元组结构体看起来跟元组很像，但是它拥有一个结构体的名称，该名称可以赋予它一定的意义。由于它并不关心内部数据到底是什么名称，因此此时元组结构体就非常适合
19. 你可以在实例化一个结构体时将它整体标记为可变的，但是 Rust 不允许我们将结构体的某个字段专门指定为可变的
20. 使用结构体字段初始化缩略语法可以减少一些重复代码
21. User {
        email: String::from("contact@im.dev"),
         ..u  
	}
22. 
	#[derive(Debug)]
	struct Rectangle {
		width: u32,
		height: u32,
	}
23. owenership of struct
    当解构一个变量时，可以同时使用 move 和引用模式绑定的方式。当这么做时，部分 move 就会发生：变量中一部分的所有权被转移给其它变量，而另一部分我们获取了它的引用。
	在这种情况下，原变量将无法再被使用，但是它没有转移所有权的那一部分依然可以使用，也就是之前被引用的那部分。

#### flow control
1. for n in 1..100
2. for (i,v) in a.iter().enumerate()
3. break 2*counter;

#### pattern-match
1. matches 
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }
2. if let Some(age) = age  // 变量遮蔽 
3. 使用 | 可以匹配多个值, 而使用 ..= 可以匹配一个闭区间的数值序列
4. @ 操作符可以让我们将一个与模式相匹配的值绑定到新的变量上
```rust
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id@3..=7,
        } => println!("id 值的范围在 [3, 7] 之间: {}", id),
        Message::Hello { id: newid@(10 | 11 | 12) } => {
            println!("id 值的范围在 [10, 12] 之间: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
```
5. match guard
```rust
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
```
6. ignore some values
```rust
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first,..,last) => {
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }
```
7. 使用模式 &mut V 去匹配一个可变引用时，你需要格外小心，因为匹配出来的 V 是一个值，而不是可变引用
```rust
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        // The type of value is &mut String
       value => value.push_str(" world!") 
    }
```
8. 引用和解引用的一些理解
```rust
我理解这里的逻辑如下：

rust中对于存储在堆内存上的数据类型，拥有其所有权的变量内存储的是指向该堆内存的指针，
例如let mut v = String::from("hello,");语句执行后v的内容就是指向堆内存数据String::from("hello,")的一个指针；
rust中的引用是对指向变量的指针而非直接指向数据的指针，例如let r = &mut v;语句执行后变量r指向变量v而非堆内存中的数据String::from("hello,")。该过程可以参考rust官方教程.
考虑语句let s = *r;，这里尝试使用*解引用，如果操作成功，那么引用r指向的变量v的所有权就会转移至变量s，进而v就无效了，但此时对变量v的引用r仍然存在，那么就会引发错误。
考虑语句let s = r.clone();，这里使用clone将堆内存数据重新复制了一份，并将该数据的所有权交给了s，整个过程中v，r一直有效，也就不会报错了。
```

#### methods and associated functions
1. &'static str , &str, String, str, &String, Box<str>, Box<&str>

#### generics
1. fn sum<T: std::ops::Add<Output=T>>(a:T, b:T) -> T{}
2. 泛型方法: 参数或{}内部用到泛型时，类型上要先提出来
```rust
impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}
impl<T, U> Point<T, U> {
    // 实现 mixup，不要修改其它代码！
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T,W>{
        return Point{
            x: self.x,
            y: other.y,
        };
    }
}
```
2. const 泛型参数
```rust
一个单独的 const 泛型参数
一个字面量 (i.e. 整数, 布尔值或字符).
一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)

fn bar<T, const M: usize>() {
    foo::<M>(); // ok: 符合第一种
    foo::<2021>(); // ok: 符合第二种
    foo::<{20 * 100 + 20 * 10 + 1}>(); // ok: 符合第三种
    
    foo::<{ M + 1 }>(); // error: 违背第三种，const 表达式中不能有泛型参数 M
    foo::<{ std::mem::size_of::<T>() }>(); // error: 泛型表达式包含了泛型参数 T
    
    let _: [u8; M]; // ok: 符合第一种
    let _: [u8; std::mem::size_of::<T>()]; // error: 泛型表达式包含了泛型参数 T
}

??? const 泛型还能帮我们避免一些运行时检查，提升性能

pub struct MinSlice<T, const N: usize> {
    pub head: [T; N],
    pub tail: [T],
}

fn main() {
    let slice: &[u8] = b"Hello, world";
    let reference: Option<&u8> = slice.get(6);
    // 我们知道 `.get` 返回的是 `Some(b' ')`
    // 但编译器不知道
    assert!(reference.is_some());

    let slice: &[u8] = b"Hello, world";

    // 当编译构建 MinSlice 时会进行长度检查，也就是在编译期我们就知道它的长度是 12
    // 在运行期，一旦 `unwrap` 成功，在 `MinSlice` 的作用域内，就再无需任何检查    
    let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
    let value: u8 = minslice.head[6];
    assert_eq!(value, b' ')
}
```
3. 打印泛型数组
```rust
fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T;N]) {
    println!("{:?}", arr);
}
```
4. const表达式的应用
**凡是要用std库feature的，都要用nightly编译。cargo +nightly**
```rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// 修复 main 函数中的错误
fn main() {
    check_size([0u8; 767]); 
    check_size([0i32; 191]);
    check_size(["hello你好"; __]); // size of &str ?
    check_size([(); __].map(|_| "hello你好".to_string()));  // size of String?
    check_size(['中'; __]); // size of char ?
}



pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
```

#### traits

#### Collections
1. s.push_str(&"world".to_string());
2. &str, String 
```rust
fn main() {  
   let mut s = String::from("hello, world");

   let slice1: &str = &s; // 使用两种方法
   assert_eq!(slice1, "hello, world");

   let slice2 = &s[..5];
   assert_eq!(slice2, "hello");
   
   print_type_of(&s);
//    let slice3: &= ____ ; 
   let slice3: &mut String = &mut s; 
   // let mut slice3 : String = String::from(s);
   //print_type_of(&s);
   slice3.push('!');
   assert_eq!(slice3, "hello, world!");

   println!("Success!")
}
```

#### Closure


#### lifeTime
1. 闭包函数的消除规则
```rust
fn fn_elision(x: &i32) -> &i32 { x }
let closure_slision = |x: &i32| -> &i32 { x };
```
2. reborrow
```rust
fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    // reborrow! 此时对`r`的再借用不会导致跟上面的借用冲突
    let rr: &Point = &*r;

    // 再借用`rr`最后一次使用发生在这里，在它的生命周期中，我们并没有使用原来的借用`r`，因此不会报错
    println!("{:?}", rr);

    // 再借用结束后，才去使用原来的借用`r`
    r.move_to(10, 10);
    println!("{:?}", r);
}
```
3. let mut s = String::with_capacity(25);
4. let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
5. vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式
6. 切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
8. Vec , dyn
```rust
    let v: Vec<Box<dyn IpAddr>>= vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
```
9. HashMap 默认使用 SipHash 1-3 哈希算法，该算法对于抵抗 HashDos 攻击非常有效。在性能方面，如果你的 key 是中型大小的，那该算法非常不错，但是如果是小型的 key( 例如整数 )亦或是大型的 key ( 例如字符串 )，那你需要采用社区提供的其它算法来提高性能。
哈希表的算法是基于 Google 的 SwissTable，你可以在这里找到 C++ 的实现，同时在 CppCon talk 上也有关于算法如何工作的演讲
10. hashmap
```rust
	let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));
```
11. let teams_map2: HashMap<_,_> = teams.into_iter().collect();
12. let teams_map2 = HashMap::from(teams);
13. player_stats.entry("health").or_insert(100);
14. player_stats.entry("health").or_insert_with(random_stat_buff);
15. 事实上，虽然我们使用了 100 容量来初始化，但是 map 的容量很可能会比 100 更多
16. map.shrink_to(50);
17. map.shrink_to_fit();
18. 第三方哈希
```rust
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
// 引入第三方的哈希函数
use twox_hash::XxHash64;


let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
hash.insert(42, "the answer");
assert_eq!(hash.get(&42), Some(&"the answer"));
```
19. 任何实现了Eq和Hash特征的类型都可以用于HashMap的key，包括(bool, int, uint, String, &str)

#### format output
1. 默认情况下，通过空格来填充字符串
2. assert_eq!(format!("Hello {1:0$}!", 5, "x"), "Hello x    !");
3. assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !");
4. 对齐输出
```rust
    // 左对齐
    println!("Hello {:<5}!", "x"); // => Hello x    !
    // 右对齐
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    // 居中对齐
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

    // 左对齐，并使用 `&` 填充
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");
```
5. 格式化补齐输出
```rust
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    /* 填空 */
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");
```
6. 格式化输出浮点数
```rust
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416 

    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");
```
7. 格式化字符串
```rust
    let s = "Hello, world!";

    println!("{0:.5}", s); // => Hello

    assert_eq!(format!("Hello {1:.0$}!", 3, "abcdefg"), "Hello abc!");
```

8. 不同进制格式化输出
```rust
    format!("{}", foo) // "3735928559"
    format!("0x{:X}", foo) // "0xDEADBEEF"
    format!("0o{:o}", foo) // "0o33653337357"
    assert_eq!(format!("0b{:b}", 27), "0b11011");
    assert_eq!(format!("0o{:o}", 27), "0o33");
    assert_eq!(format!("0x{:x}", 27), "0x1b");
    assert_eq!(format!("0x{:X}", 27), "0x1B");

    println!("{:x}!", 27); // 没有前缀的十六进制 => 1b

    println!("{:#010b}", 27); // 使用 0 来填充二进制，宽度为 10 => 0b00011011
```
9. 指数
```rust
    // 指数
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9

    // 指针地址
    let v= vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x600002324050

    // 转义
    println!("Hello {{}}"); // => Hello {}
```
10. 捕获环境中的值
```rust
    let person = get_person();
    println!("Hello, {person}!");
```

#### documentation
1. cargo new --lib doc-comments
2. cargo doc --open
3. 注意: 必须要将包、模块注释放置在包根或模块文件的最顶部
4. 文档注释会被解析为 HTML 文件，并支持 Markdown 语法。
5. 块文档注释 /** ... */
6. [`Option`] 
7. 注释文档中不显示 #

#### type transform
1. f32 -> char  // error                 f32->u8->char  // ok
2. #[allow(overflowing_literals)]
3. 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
4. 类型转换：安全和性能
5. 其他：no_inline   #[doc(hidden)]
```rust
fn main() {
    assert_eq!(1000 as u16, 1000);

    assert_eq!(1000_u16 as u8, (1000_u16 % (u8::MAX as u16 + 1)) as u8);

    // 事实上，之前说的规则对于正整数而言，就是如下的取模
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, u8::MAX);
    

    // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
    assert_eq!(300.1_f32 as u8, u8::MAX);
    assert_eq!(-100.1_f32 as u8, u8::MIN);
    

    // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
    // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
    // 总之，请小心使用
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
```
1. unsafe的指针操作
```rust
// 填空
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize; 
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 指向 values 数组中的第二个元素
    unsafe {
        // 将第二个元素加 1
        *p2 += 1;
    }
    
    assert_eq!(values[1], 3);

    println!("Success!")
}
```
6. 困惑的一个场景
```rust
fn main() {
    let arr :[u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 13)
    }
}
```

7. use std::convert::TryInto;