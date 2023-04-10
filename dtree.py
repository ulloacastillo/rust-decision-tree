
import timeit


import pandas as pd
import time
from sklearn.tree import DecisionTreeClassifier

df = pd.read_csv('iris.csv', header=None)

X = df.iloc[:, :4]
y = df.iloc[:, 4].values

model = DecisionTreeClassifier(criterion='gini')

s = time.time()

time.sleep(2)
model.fit(X, y)

print((time.time()-s)*1000)
