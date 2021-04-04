## Notes

#### Development

- `ctrl + Q`: show docs for symbol under cursor
- `ctrl + K`: VCS (git) commit dialog

#### Testing

- First, install: `cargo install cargo-watch`
- Then, from the project's root directory run `cargo watch -x test` to automatically rerun all tests on save.

#### Further Reading

- https://cheats.rs/ "Cheat sheet" with links to more details in the online rust books

## TODO

- [ ] in Rust, can we make Vector/Point types that "inherit" from tuple but only implement relevant operations on each
  one?
- [ ] Learn how to split things into modules.
  See https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
- [ ] Learn how to split projects into crates that are part of a
  workspace https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

## Done

- [x] add 'clippy' linter to project

## Book Progress

- ✅ Chapter 1: Tuples, points and vectors
- Chapter 2: Drawing on a canvas
- Chapter 3: Matrices
- Chapter 4: Matrix transformations
- Chapter 5: Ray-Sphere intersections
- Chapter 6: Light and Shading
- Chapter 7: Making a Scene
- Chapter 8: Shadows
- Chapter 9: Planes
- Chapter 10: Patterns
- Chapter 11: Reflection
- Chapter 12: Cubes
- Chapter 13: Cylinders
- Chapter 14: Groups
- Chapter 15: Triangles
- Chapter 16: Constructive Solid Geometry (CSG)
- Chapter 17: Next Steps
