dev:
  @watchexec --project-origin . --clear --restart --ignore target --watch src cargo run

test-arm:
  curl -X POST http://127.0.0.1:3000/asm \
    -H 'Content-Type: application/json' \
    -H 'Authorization: Bearer deadbeef-dead-beef-dead-beefdeadbeef' \
    -d '{"asm":".global _start\n_start:\nmov x0, #33\nmov x1, #12\nadd x0, x0, x1\nmov x16, #1\nsvc 0\n"}'

