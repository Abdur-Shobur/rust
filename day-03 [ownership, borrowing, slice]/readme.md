**Summary Day-03**

# Ownership

_What is Ownership?_

- Rust-এ garbage collector নেই।
- Rust মেমোরি ম্যানেজ করে Ownership নিয়মে।

  **Ownership-এর ৩টি নিয়ম:**

- প্রতিটি value-এর একজন owner থাকে
- Owner scope থেকে বের হলে value অটোমেটিক free হয়
- একটা value-এর এক সময় একজনই owner থাকতে পারে

# Move (Value এক জায়গা থেকে অন্য জায়গায় চলে যায়)

- normal type থেকে এই copy হয়, move হয় না।

# Borrowing (ধার করা)

- value ব্যবহার করা হয়,
- মালিকানা নিজের কাছে নেওয়া হয় না।
- mutable , immutable borrow
-

# Slices

- String-এর নির্দিষ্ট অংশ borrow করা — এটাকে slice বলে।
- array slice
