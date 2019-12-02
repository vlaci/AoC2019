module Lib

@enum OPCODE begin
  ADD = 1
  MUL = 2
  HALT = 99
end

function runprogram(code::Array)
    code = copy(code)
    idx = 1
    while true
        op = OPCODE(code[idx])
        if op == HALT
            break
        end
        op1 = code[code[idx + 1] + 1]
        op2 = code[code[idx + 2] + 1]
        storeat = code[idx + 3] + 1
        if op == ADD
            code[storeat] = op1 + op2
        elseif op == MUL
            code[storeat] = op1 * op2
        end
        idx += 4
    end
    code
end

end
