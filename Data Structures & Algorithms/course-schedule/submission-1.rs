const PRE: usize = 0;
const COURSE: usize = 1;

#[derive(Clone, Debug)]
enum Status {
    Done,
    InProgress,
    Todo
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut graph = vec![Vec::new(); num_courses];
        let mut status = vec![Status::Todo; num_courses];

        for edge in prerequisites.iter() {
            graph[edge[PRE] as usize].push(edge[COURSE] as usize);
        }

        (0..num_courses).all(|course| !Self::dfs(course, &mut status, &graph))
    }

    fn dfs(course: usize, status: &mut Vec<Status>, graph: &Vec<Vec<usize>>) -> bool {
        match status[course] {
            Status::Done => false,
            Status::InProgress => true,
            _ => {
                status[course] = Status::InProgress;
                if graph[course].iter().any(|&next_course| Self::dfs(next_course, status, graph)) {
                    return true;
                }
                status[course] = Status::Done;
                false
            }
        }
    }
}
