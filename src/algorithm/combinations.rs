/*
  计算组合数
  递推公式 C[n][m] = c[n-1][m-1] + c[n-1][m]
*/

// n>m
fn cal_c(n: usize, m: usize) -> Vec<Vec<i32>> {
    let mut c = vec![vec![0; m]; n];
    c[1][0] = 1;
    c[1][1] = 1;
    for i in 2..n {
        c[i][0] = 1;
        for j in 1..m {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    c
}
