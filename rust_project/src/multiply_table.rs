pub fn exec() {
    let table = multiply_table(1024, 1024);
    if table[0][0] == 0 {
        print!("Ops!")
    }
}

type Table = Vec<Vec<usize>>;

fn multiply_table(x_length: usize, y_length: usize) -> Table {
    let mut table: Table = Vec::with_capacity(x_length);

    for x in 0..x_length {
        let mut line = Vec::with_capacity(y_length);
        for y in 0..y_length {
            line.push((x + 1) * (y + 1))
        }
        table.push(line);
    }

    table
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let table = multiply_table(256, 256);
        assert_eq!(table[255][255], 65536);
    }
}

