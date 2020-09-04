#[derive(Debug, Clone)]
struct Activity {
    start: i32,
    end: i32,
}

fn greedy_activity_selection(activities: Vec<Activity>) -> Vec<Activity> {
    let mut current_activity: Activity;
    let mut subset: Vec<Activity> = Vec::new();

    subset.push(activities[0].clone());
    current_activity = activities[0].clone();

    for activity in activities.into_iter() {
        if activity.end > current_activity.end && activity.start >= current_activity.end {
            subset.push(activity.clone());
            current_activity = activity.clone();
        }
    }

    subset
}

fn recursive_activity_selection(activities: Vec<Activity>) -> Vec<Activity> {
    if activities.is_empty() {
        let empty: Vec<Activity> = Vec::new();
        empty
    } else {
        let a = vec![activities.clone().pop().unwrap()];

        let first_end = a[0].end;

        let b: Vec<Activity> = a.into_iter()
            .chain(
                activities[1..]
                .into_iter()
                .cloned()
                .filter(|x| x.start >= first_end)
                ).collect();

        b
    }
}


fn main() {
    let mut activities: Vec<Activity> = Vec::new();

    let start_times = [1, 3, 0, 5, 3, 5, 6, 8, 8, 2, 12];
    let end_times = [4, 5, 6, 7, 9, 9, 10, 11, 12, 14, 16];

    for i in 0..start_times.len() {
        let start = start_times[i];
        let end = end_times[i];

        activities.push(Activity {
            start,
            end
        });
    }

    println!("{:?}", activities);

    let subset = greedy_activity_selection(activities);

    println!("{:?}", subset);

    println!("Max subset: {:?}", greedy_activity_selection(activities));
}
