// define ROTATE(x, y)  (((x)>>(y)) | ((x)<<(32-(y))))
pub fn rotate32(y: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 32 - y..32 {
        res.push(i);
    }
    for i in 0..32 - y {
        res.push(i);
    }
    res
}