import pandas as pd
import numpy as np
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import LSTM, Dense

data = pd.read_csv('cursor_positions.csv', header=None, names=['x', 'y'])
scaler = MinMaxScaler()
data_scaled = scaler.fit_transform(data)

def create_sequences(data, seq_length):
    xs, ys = [], []
    for i in range(len(data) - seq_length):
        xs.append(data[i:i+seq_length])
        ys.append(data[i+seq_length])
    return np.array(xs), np.array(ys)

seq_length = 10
X, y = create_sequences(data_scaled, seq_length)

split = int(0.8 * len(X))
X_train, X_test = X[:split], X[split:]
y_train, y_test = y[:split], y[split:]

model = Sequential([
    LSTM(50, return_sequences=True, input_shape=(seq_length, 2)),
    LSTM(50),
    Dense(2)
])
model.compile(optimizer='adam', loss='mse')
model.fit(X_train, y_train, epochs=10, batch_size=32, validation_split=0.2)

def predict_next_position(current_sequence):
    current_sequence_scaled = scaler.transform(current_sequence)
    prediction_scaled = model.predict(np.array([current_sequence_scaled]))
    prediction = scaler.inverse_transform(prediction_scaled)[0]
    return prediction

print("Enter 'q' to quit.")
while True:
    try:
        current_x = float(input("Enter current X coordinate: "))
        current_y = float(input("Enter current Y coordinate: "))
    except ValueError:
        print("Exiting...")
        break

    last_positions = data.iloc[-9:].values.tolist()
    last_positions.append([current_x, current_y])

    predicted_position = predict_next_position(last_positions)
    print(f"Predicted next position: X={predicted_position[0]:.2f}, Y={predicted_position[1]:.2f}")
    print()