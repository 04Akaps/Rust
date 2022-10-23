use std::collections::VecDeque;
// ring buffer
// https://doc.rust-lang.org/std/collections/struct.VecDeque.html

mod random;
use random::random_range;

pub type Position = (usize, usize);

#[derive(Debug)]
pub enum Direction {
    Top, Right, Bottom, Left
}
#[derive(Debug)]
pub struct SnakeGame {
    width : usize,
    height : usize,
    snake : VecDeque<Position>,
    direction : Direction,
    food : Position,
    game_over : bool
}

impl SnakeGame {
    pub fn new(width : usize, height : usize) -> Self {
        Self {
            width,
            height,
            snake : [((width - 2).max(0), (height / 2).max(0))].into_iter().collect(),
            // https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
            // 쉽게 말하면 둘 모드 array를 순회하는 메서드 이지만 into_iter()메서드는 다양한 타입을 return이 가능하지만, iter() 같은 경우에는 레퍼런스 타입만 return 된다.

            // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
            // collect 메서드는 iter타입을 다시  합쳐 준다.
            direction : Direction::Left,
            food : (2.min(width - 1), height / 2),
            game_over: false
        }
    }

    pub fn change_direction(&mut self, direction : Direction) {
        match (&self.direction, direction){
            (Direction::Top, Direction::Top) |
            (Direction::Top, Direction::Bottom) |
            (Direction::Right, Direction::Right)|
            (Direction::Right, Direction::Left) |
            (Direction::Left, Direction::Right) |
            (Direction::Left, Direction::Left) |
            (Direction::Bottom, Direction::Top) |
            (Direction::Bottom, Direction::Bottom)  => {}
            (_, direction) => self.direction = direction,
        }
    }

    pub fn is_valid (&self, (x,y): Position) -> bool {
        // 위치 이동이 가능한지를 확인하는 함수
        // 정확하게는 자신의 body에 부딪쳤는지를 확인하는 함수
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        // move를 담당하는 함수

        if self.game_over && self.snake.len() == 0 {
            // 만약 game_over 되어 있는 상태라면 작업하지 않는다.
            return;
        };

        let head = self.snake.get(0);
        // get은 option타입으로 return 된다.
        // 앞서 self.snake.len() == 0 일떄를 고려했기 때문에 굳이 get을 사용하지 않아도 된다.
        // 절대적으로 panic이 일어나지 않기 떄문에
        // 하지만 이 코드가 좀더 배우는 부분이 많다고 생각을 하기 떄문에 이렇게 작성을 냅두었다.

        let new_head = head.map(|&(x,y) |  match &self.direction {
            // 그후 match를 확인한다.
            // map우리가 일반적으로 아는 메서드랑은 다르다.
            // Option에 사용이 되는 메서드 이며 해당 메서드의 타입을 바꾸어 준다.

            // docs : Converts an Option<String> into an Option<usize>, consuming the original
            Direction::Top => (x, y-1),
            Direction::Right => (x+1, y),
            Direction::Left => (x-1, y),
            Direction::Bottom => (x, y-1),

            // 이후 return 같은 경우에는 레퍼런스를 사용하여 new_head에 값을 return 해주면 된다.
        });

        // 그후 뭐 벗겨난 값이 존재 할떄에 Ring 타입의 array값을 수정해 준다.
        if let Some(new_head) = new_head {
            if !self.is_valid(new_head)  || self.snake.contains(&new_head) {
                // 자신의 몸통에 부딪쳤을떄를 검증
                self.game_over = true;
            }else {
                if new_head != self.food {
                    // 만약 food의 위치에 없다면 뒤에 있는 꼬리를 잘라주어야 한다.
                    self.snake.pop_back();
                } else{
                    // 하지만 만약 food에 들어갔다면 어떠한 처리를 해주어야 한다.
                    let free_positoin = (0..self.height).flat_map(
                        |y| (0..self.width).map(
                            move |x| (x,y))).filter(
                                |pos| !self.snake.contains(pos)).collect::<Vec<_>>();

                    if free_positoin.is_empty() {
                        self.game_over = true;
                        return;
                    }

                    self.food = free_positoin[random_range(0, free_position.len())];
                }
                // 뒤에 있는 꼬리는 자르고 앞에 있는 머리는 전진한다.
                self.snake.push_front(new_head);
            }
        }
    }
}


// cargo test -- --show-output
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10,10));
    }
}