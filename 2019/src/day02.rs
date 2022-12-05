pub(crate) fn part_1(input: &str) -> String {
    let ins = get_ins(input);

    let (noun, verb) = if ins.len() <= 12 { (9, 10) } else { (12, 2) };

    let res = calculate_program(ins, noun, verb);

    res.to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let ins = get_ins(input);

    if ins.len() <= 12 {
        return "Part 2 not applicable for example".to_owned();
    }

    let (noun, verb) = find_noun_and_verb(ins).unwrap();

    (100 * noun + verb).to_string()
}

fn get_ins(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|n| str::parse::<usize>(n).unwrap())
        .collect()
}

fn calculate_program(mut ins: Vec<usize>, noun: usize, verb: usize) -> usize {
    ins[1] = noun;
    ins[2] = verb;

    for ip in (0..ins.len()).step_by(4) {
        let opcode = ins[ip];
        let addr_1 = ins[ip + 1];
        let addr_2 = ins[ip + 2];
        let res_addr = ins[ip + 3];

        log::debug!("opcode: {opcode}, addr_1: {addr_1}, addr_2: {addr_2}, res_addr: {res_addr}");

        if opcode == 1 {
            ins[res_addr] = ins[addr_1] + ins[addr_2];
        } else if opcode == 2 {
            ins[res_addr] = ins[addr_1] * ins[addr_2];
        } else if opcode == 99 {
            break;
        }

        log::debug!("result: {}", ins[res_addr]);
    }

    ins[0]
}

fn find_noun_and_verb(ins: Vec<usize>) -> Option<(usize, usize)> {
    for noun in 0..100 {
        for verb in 0..100 {
            let res = calculate_program(ins.clone(), noun, verb);
            if res == 19690720 {
                return Some((noun, verb));
            }
        }
    }

    None
}
