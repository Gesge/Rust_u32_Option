// 题目二：实现一个函数，为u32类型的整数集合请求和，参考类型为&[u32]，返回类型为Option，溢出时返回None；
fn sumu32(list: &[u32]) -> Option<u32> {
    let mut result: u64 = 0;

    for v in list {
        let n = *v as u64;
        if (result + n) < result {
            return None;
        } else {
            result += n;
        }
    }

    return Some(result as u32);
} 
 
fn main() { 
    // 构造测试用的u32整数集合 
    let integers: &[u32] = &[1, 2, 3, 4]; 
    // 调用sumu32这个函数计算结果并打印出来 
    println!("{:#?}", sumu32(integers)); // 结果是10 

    // 构造更大的数字，以触发溢出情况。 
    let bigints: &[u32] = &[2u32.pow(31),2u32.pow(32)];
    println!("{:#?}", sumu32(bigints)); // 得到None 以代表溢出的失败情况     
}
