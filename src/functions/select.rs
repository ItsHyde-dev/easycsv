pub fn get_selected_header_position_list(headers: &Vec<String>, select: Vec<String>) -> Vec<usize> {
    let mut pos_list: Vec<usize> = Vec::new();

    for col in select {
        let pos = headers.iter().position(|x| *x == col);

        if let Some(pos) = pos {
            pos_list.push(pos);
        }
    }

    return pos_list;
}
