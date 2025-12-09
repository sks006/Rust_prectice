# What the for loop actually receives
### arr.iter().enumerate() does not produce values like:

```
0, 1, 2, ...
7, 14, 21, ...

```
### It produces tuples each time:
```
(index, &element)

(0, &7)
(1, &14)
(2, &21)
(3, &28)
(4, &35)

```
## 🔧 2. Syntax → Rule → Purpose → Usage

### ✔ Syntax (pattern matching in loop)
```
for (index, &value) in iterator

```
#### ✔ Rule Rust’s for loop destructures each item coming from the iterator using pattern matching.
A tuple (a, b) can be destructured by writing (x, y).
#### This is identical to:
```
let (index, &value) = (0, &7);

```
### ✔ Purpose
#### You extract both parts of the tuple directly inside the loop, so you don’t have to write:
```
let pair = iterator.next();
let index = pair.0;
let value = pair.1;

```
### 🧠 3. What does &value mean?
#### Each item of arr.iter() is &i32, so .enumerate() yields:
```
(usize, &i32)

//But in your loop you wrote:
(value is i32, not &i32)
for (index, &value) in arr.iter().enumerate()

// &value = “match a reference and bind the thing inside it.”

//So: 
(2, &21)   //matches pattern   (index, &value)
/*
→ index = 2
→ value = 21 (dereferenced)

*/

```