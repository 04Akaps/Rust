use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Age(u64);

#[derive(Component)]
struct Person;

// struct Person;
// 보통 일반적인 경우라면 이렇게 사용을 합니다.
// 하지만 만약 Person이라는 값에 Name이라는 프로퍼티가 있고, 이후 Dog라는 값에도 Name이라는 값이 있을 수 있습니다.
// 그럼 Name이 두개 선언이 되어서 중복이 발생하니깐 위와 같이 활용이 가능합니다.

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        println!("build is right");

        app.add_startup_system(add_people)
            .add_system(greet_name)
            .add_system(greet_age);
    }
}

fn main() {
    // 이렇게 따로따로 시스템을 설정하여 사용도 가능하지만 다음과 같이 작성하면 좀더 간결한 코드가 됩니다.

    // App::new()
    //     // .add_plugins(DefaultPlugins)
    //     // DefaultPlugins을 사용하면 event loop을 동작시키기 떄문에 계속해서 루프가 돌게 됩니다.
    //     .add_plugin(HelloPlugin)
    //     .add_startup_system(add_people)
    //     .add_system(hello_world)
    //     .add_system(greet_name)
    //     .add_system(greet_age)
    //     .run();

    App::new()
        .add_plugin(HelloPlugin)
        // time_test함수 주석 참고
        // .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        // .add_system(time_test)
        .add_system(hello_world)
        .run();

    /*
        .add_plugins(DefaultPlugins)

        위에 있는 코드와 아래 있는 코드는 같습니다.
        .add_plugin(CorePlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin::default())
    */
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()))
        .insert(Age(3));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()))
        .insert(Age(5));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()))
        .insert(Age(10));
}

fn hello_world() {
    println!("hello world!");
}

fn greet_name(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

struct GreetTimer(Timer);

fn time_test(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // 특정 시간마다 데이터 활용하기
    // Res나 &mut Res는 특정 resources에 읽거나 쓰는 것을 허용해 줍니다.

    // docs기준으로 시간을 설정하여 일정 시간마다 값을 출력하는 것이라고 하는데 버전이 달라졌는데 업데이트가 되어서 동작하지 않음
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("timer :  {}!", name.0);
        }
    }
}

fn greet_age(query: Query<&Age, With<Person>>) {
    for age in query.iter() {
        println!("MyAge {}!", age.0);
    }
}
