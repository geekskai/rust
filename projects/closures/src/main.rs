// 类似 JS: const ShirtColor = { Red: 'Red', Blue: 'Blue' }
// 或 TypeScript: enum ShirtColor { Red = 'Red', Blue = 'Blue' }
// #[derive(Debug, Clone)] 相当于给对象添加 toString() 和深拷贝方法
#[derive(Debug, Clone)]
enum ShirtColor {
    Red,  // 就像 ShirtColor.Red
    Blue, // 就像 ShirtColor.Blue
}

// 类似 JS: class Inventory { constructor(shirts) { this.shirts = shirts; } }
// 或对象: { shirts: [ShirtColor.Red, ShirtColor.Blue] }
struct Inventory {
    shirts: Vec<ShirtColor>, // Vec<T> 就像 JS 的 Array<T>
}

// 类似 JS: class Inventory { ... } 中的方法定义
impl Inventory {
    // JS 等价: giveaway(userPreference) { return userPreference || this.mostStocked(); }
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // &self 相当于 JS 的 this
        // Option<ShirtColor> 相当于 ShirtColor | null | undefined
        // unwrap_or_else() 相当于 || 运算符，提供默认值
        // || self.most_stocked() 相当于 () => this.mostStocked()
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    // JS 等价: mostStocked() { ... }
    fn most_stocked(&self) -> ShirtColor {
        // let mut 相当于 JS 的 let（可变变量）
        let mut num_red = 0;
        let mut num_blue = 0;

        // for...in 循环，&self.shirts 相当于 this.shirts
        // & 表示借用（引用），不获取所有权
        for color in &self.shirts {
            // match 相当于 JS 的 switch，但更强大
            // 类似: if (color === ShirtColor.Red) { numRed++; }
            match color {
                ShirtColor::Red => num_red += 1,   // case ShirtColor.Red:
                ShirtColor::Blue => num_blue += 1, // case ShirtColor.Blue:
            }
        }

        // 三元运算符: return numRed > numBlue ? ShirtColor.Red : ShirtColor.Blue;
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

// 主函数，相当于 React 组件或 JS 的入口函数
fn main() {
    // 创建实例，类似: const store = new Inventory([...])
    // vec![] 宏相当于 JS 的 [...] 数组字面量
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    // Some() 表示有值，相当于 JS 的 ShirtColor.Red（非 null/undefined）
    let user_pref1 = Some(ShirtColor::Red);

    // .clone() 创建副本，避免所有权转移
    // 类似 JS: const giveaway1 = store.giveaway({...userPref1})
    let giveaway1 = store.giveaway(user_pref1.clone());

    // println! 相当于 console.log()
    // {:?} 相当于 JSON.stringify() 或对象的 toString()
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    // None 表示没有值，相当于 JS 的 null 或 undefined
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2.clone());

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
