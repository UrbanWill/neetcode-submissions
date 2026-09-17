const PRE: usize = 1;
const COURSE: usize = 0;

#[derive(Debug, Clone)]
enum Status {
    Todo,
    Done,
    InProgress
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut status = vec![Status::Todo; num_courses];
        let mut graph = vec![Vec::new(); num_courses];

        for edge in prerequisites.iter() {
            graph[edge[COURSE] as usize].push(edge[PRE] as usize);
        }

        (0..num_courses).all(|course| !Self::has_cycle(course, &mut status, &graph))
    }

    fn has_cycle(course: usize, status: &mut Vec<Status>, graph: &Vec<Vec<usize>>) -> bool {
        match status[course] {
            Status::Done => false,
            Status::InProgress => true,
            _ => {
                status[course] = Status::InProgress;

                if graph[course].iter().any(|&next_course| Self::has_cycle(next_course, status, graph)) {
                    return true;
                }

                status[course] = Status::Done;
                false
            }
        }   
    }
}
