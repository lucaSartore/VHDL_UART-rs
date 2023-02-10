#this program will generate the vhdl code to process the inputs
RES_X: int  = 16
RES_Y: int = 16

NAME_INPUT = 'input'
NAME_OUTPUT = 'output'

f = open("vddl_generated_code.txt", "w")

for x in range(RES_X):
    for y in range(RES_Y):

        f.write(
            f"output_X{x}_Y{y}_gray <= input_X{x}_Y{y}_r when input_X{x}_Y{y}_r >= input_X{x}_Y{y}_b and input_X{x}_Y{y}_r >= input_X{x}_Y{y}_g\
else input_X{x}_Y{y}_g when input_X{x}_Y{y}_g >= input_X{x}_Y{y}_b else input_X{x}_Y{y}_b;\n"
        )

f.close()

