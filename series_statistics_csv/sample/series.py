import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

filepath = 'sample.csv'
df = pd.read_csv(filepath, sep=',', index_col=0)
time=np.array(list(df['time']))
upper=np.array(list(df['upper']))
lower=np.array(list(df['lower']))
mean=np.array(list(df['mean']))
print(df)

# plt.fill_between
alpha=0.5
label_name_x = 'Time[s]'
label_name_y = 'Amplitude'
plt.xlabel(label_name_x, rotation='horizontal')
plt.ylabel(label_name_y, rotation='vertical')
plt.plot(time, mean, color='green', label='mean')
plt.plot(time, upper, color='blue', label='mean + std.dev.')
plt.plot(time, lower, color='red', label='mean - std.dev.')
plt.grid()
plt.legend(loc='upper right')
plt.fill_between(time, upper, lower, label='sample', facecolor='skyblue', alpha=alpha, where=upper>lower)
plt.show()