#[derive(Debug)]
pub struct Stack<T> {
    size: usize, // 栈大小
    data: Vec<T>, // 栈内数据
}

impl<T> Stack<T> {
    // 初始化空栈
   pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // 清空栈
    pub fn clear(&mut self){
        self.size = 0;
        self.data.clear();
    }

    // 插入数据
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    // 弹出栈顶元素
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.size -= 1;
        self.data.pop()
    }

    // 返回栈顶元素和可变引用
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.data.get(self.size - 1)
    }


    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }

        self.data.get_mut(self.size - 1)
    }

}

// 迭代器1： 栈改变，成为迭代器
pub  struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        }else {
            None
        }
    }
}
// 迭代器2： 栈不变，得到不可变迭代器
pub struct Iter<'a, T: 'a>{stack: Vec<&'a T>, }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

// 迭代器3： 栈不变，得到可变迭代器
pub struct IterMut<'a, T: 'a>{stack: Vec<&'a mut T>}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}