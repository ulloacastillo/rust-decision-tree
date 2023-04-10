import subprocess
import statistics

# Define a list of input values
n = 10
# Initialize an empty list to store the outputs
outputs = []

# Loop over the input values and execute the command
for i in range(n):
    # Execute the command and capture the output
    output = subprocess.check_output(
        ['cargo run --release'], shell=True, text=True)

    # Append the output to the list of outputs
    outputs.append(float(output.strip('ms\n')))
    # outputs.append(output)


# Print the outputs
print(f"Mean {sum(outputs)/n} ms +- {statistics.stdev(outputs)} ms")
