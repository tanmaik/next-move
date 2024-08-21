import pandas as pd
import numpy as np
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import LSTM, Dense

# load the data
data = pd.read_csv('cursor_positions.csv', header=None, names=['x', 'y'])

# normalize the data
scaler = MinMaxScaler()
data[['x', 'y']] = scaler.fit_transform(data[['x', 'y']])

# create sequences
def create_sequences(data, seq_length):
    xs, ys = [], []
    for i in range(len(data) - seq_length):
        x = data.iloc[i:(i + seq_length)].values
        y = data.iloc[i + seq_length].values
        xs.append(x)
        ys.append(y)
    return np.array(xs), np.array(ys)

seq_length = 10
X, y = create_sequences(data, seq_length)

# split into training and testing sets
split = int(0.8 * len(X))
X_train, X_test = X[:split], X[split:]
y_train, y_test = y[:split], y[split:]

# build the lstm model
model = Sequential()
model.add(LSTM(50, return_sequences=True, input_shape=(seq_length, 2)))
model.add(LSTM(50))
model.add(Dense(2))

model.compile(optimizer='adam', loss='mse')

# train the model
model.fit(X_train, y_train, epochs=10, batch_size=32, validation_split=0.2)

# evaluate the model
loss = model.evaluate(X_test, y_test)
print(f'test loss: {loss}')