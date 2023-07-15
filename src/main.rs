/**
  * @Author: mailset
  * @Date: 2023/7/14 17
  * LiveShare别乱搞
  */

use std::io;
use regex::Regex;

fn main() {
    println!("就你会开字母是吧😅");
    let mut words1 = Vec::new();
    let mut index = String::new();
    println!("请输入有多少个单词:");
    io::stdin()
        .read_line(&mut index)
        .expect("你tm能好好输一个数字吗😅");
    let index: u32 = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => 114514,
    };
    for i in 1..=index {
        let mut astr = Default::default();
        println!("请输入单词{}:", i);
        io::stdin()
            .read_line(&mut astr)
            .expect("在那里整什么花活呢");
        let astr= astr.replace("\n", "");
        words1.push(astr);
    }
    // println!("{:?}", words1);
    let words_hidden = hidden_words(words1.clone());
    output_vec(words_hidden);
} 

/**
  * 草你妈Rust这一段我卡了一个小时
  */
// 把vec中的字符转换成*
fn hidden_words(vec: Vec<String>) -> Vec<String> {
    // 引用Regex
    let mut retvec = Vec::new();
    let re = Regex::new(r"[A-Za-z0-9]").unwrap();
    for i in 0..vec.len() {
        //提取Some中的内容
        let tstr: &str = match vec.get(i) {
            None => "",
            Some(i) => i.as_str()
        };
        // 替换
        let astr = re.replace_all(tstr, "*");
        retvec.push(astr.to_string());
    }
    retvec
}

/**
  * 这下吸取经验了捏
  */
// 输出屏蔽过的内容
fn output_vec(vec: Vec<String>) -> () {
    for i in 0..vec.len() {
        let astr = match vec.get(i) {
            None => "",
            Some(i) => i
        };
        println!("{}. {}", i + 1, astr);
    }
}

fn unhidden(ch: u8, vec: Vec<String>)
// 好困 妈的终于好了
// 先試試？（
// 试了，效果非常棒，出人意料的舒服