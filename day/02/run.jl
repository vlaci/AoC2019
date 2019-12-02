include("src/Lib.jl")

code = open("$(@__DIR__)/input") do f
    s = read(f, String)
    map(l -> parse(Int64, l), split(s, ","))
end

code[1+1] = 12
code[2+1] = 2
result = Lib.runprogram(code)[1]

println("The final state of the program is $(result)")

function part2()
result = undef
for noun = 0:99
    for verb = 0:99
        code[1+1] = noun
        code[2+1] = verb
        res = Lib.runprogram(code)[1]

        if res == 19690720
            result = 100 * noun + verb
            return result
        end
    end
end
end

println("The input is $(part2())")