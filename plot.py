import seaborn as sns
import pandas as pd
import matplotlib.pyplot as plt

sns.set_theme(style='ticks')

df = pd.read_csv("results.csv")
df = df.round(3)

sns.lineplot(data=df, x="type", y="asgs", hue="size", linewidth=2.5, markersize=8)

plt.show()
