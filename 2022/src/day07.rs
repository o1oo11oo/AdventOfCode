use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{digit1, line_ending, space1},
    combinator::{map_res, opt},
    multi::many1,
    sequence::{delimited, terminated},
    Finish, IResult,
};

pub(crate) fn part_1(input: &str) -> String {
    let (_, tree) = parse_dir(input, "/").finish().unwrap();

    tree.dirlist()
        .iter()
        .map(|d| d.size())
        .filter(|s| s <= &100000)
        .sum::<u32>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let (_, tree) = parse_dir(input, "/").finish().unwrap();

    const TOTAL_SIZE: u32 = 70000000;
    const NEEDED_SPACE: u32 = 30000000;
    let empty_space = TOTAL_SIZE - tree.size();
    let needed_to_free = NEEDED_SPACE - empty_space;

    let mut list = tree.dirlist();
    list.sort_by_key(|i| i.size());
    let to_delete = list.iter().find(|d| d.size() >= needed_to_free).unwrap();

    to_delete.size().to_string()
}

#[derive(Debug)]
enum DirItem<'a> {
    Dir(Dir<'a>),
    File(File<'a>),
}

impl<'a> DirItem<'a> {
    fn size(&self) -> u32 {
        match self {
            DirItem::Dir(dir) => dir.size(),
            DirItem::File(file) => file.size,
        }
    }
}

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    items: Vec<DirItem<'a>>,
}

impl<'a> Dir<'a> {
    fn size(&self) -> u32 {
        self.items.iter().map(|i| i.size()).sum()
    }

    fn dirlist(&self) -> Vec<&Dir<'a>> {
        self.items
            .iter()
            .filter_map(|i| match i {
                DirItem::Dir(dir) => Some(dir),
                _ => None,
            })
            .flat_map(|d| d.dirlist())
            .chain(std::iter::once(self))
            .collect()
    }
}

#[derive(Debug)]
struct File<'a> {
    _name: &'a str,
    size: u32,
}

fn parse_dir<'a>(input: &'a str, dir_name: &'a str) -> IResult<&'a str, Dir<'a>> {
    let (input, name) = delimited(tag("$ cd "), tag(dir_name), tag("\n$ ls\n"))(input)?;
    let (mut input, mut items) = many1(alt((parse_dirname, parse_file)))(input)?;

    for dir in items.iter_mut().filter_map(|i| match i {
        DirItem::Dir(dir) => Some(dir),
        _ => None,
    }) {
        let (inner_input, inner_item) = parse_dir(input, dir.name)?;
        input = inner_input;
        *dir = inner_item;
    }

    let (input, _) = opt(tag("$ cd ..\n"))(input)?;

    Ok((input, Dir { name, items }))
}

fn parse_dirname(input: &str) -> IResult<&str, DirItem<'_>> {
    let (input, name) = delimited(tag("dir "), take_until1("\n"), line_ending)(input)?;

    Ok((
        input,
        DirItem::Dir(Dir {
            name,
            items: Vec::new(),
        }),
    ))
}

fn parse_file(input: &str) -> IResult<&str, DirItem<'_>> {
    let (input, size) = map_res(terminated(digit1, space1), str::parse)(input)?;
    let (input, name) = terminated(take_until1("\n"), line_ending)(input)?;

    Ok((input, DirItem::File(File { _name: name, size })))
}
