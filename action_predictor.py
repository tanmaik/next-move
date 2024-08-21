import pandas as pd
import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import LabelEncoder
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import accuracy_score
import pyautogui
import time

def load_data(file_path):
    df = pd.read_csv(file_path)
    le = LabelEncoder()
    df['Action'] = le.fit_transform(df['Action'])
    return df, le

def create_features(df):
    features = []
    labels = []
    window_size = 10

    for action in df['Action'].unique():
        action_data = df[df['Action'] == action]
        for i in range(0, len(action_data) - window_size, window_size):
            window = action_data.iloc[i:i+window_size]
            feature = window[['X', 'Y']].values.flatten()
            features.append(feature)
            labels.append(action)

    return np.array(features), np.array(labels)

def train_model(X, y):
    X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)
    model = RandomForestClassifier(n_estimators=100, random_state=42)
    model.fit(X_train, y_train)
    
    y_pred = model.predict(X_test)
    accuracy = accuracy_score(y_test, y_pred)
    print(f"Model accuracy: {accuracy:.2f}")
    
    return model
def predict_user_action(model, le, window_size=10):
    print("Recording cursor movements for 5 seconds.")
    
    cursor_positions = []
    last_position = None
    
    start_time = time.time()
    while time.time() - start_time < 5:
        x, y = pyautogui.position()
        current_position = [x, y]
        
        if current_position != last_position:
            cursor_positions.append(current_position)
            last_position = current_position
        
        time.sleep(0.01)  # Record every 10 ms
    
    if len(cursor_positions) < window_size:
        print("Not enough cursor movements captured.")
        return
    
    feature = np.array(cursor_positions[-window_size:]).flatten()
    prediction = model.predict([feature])[0]
    action = le.inverse_transform([prediction])[0]
    
    # Get confidence scores
    probabilities = model.predict_proba([feature])[0]
    confidence_scores = {le.inverse_transform([i])[0]: prob for i, prob in enumerate(probabilities)}
    
    print(f"Predicted action: {action}")
    print("Confidence scores:")
    for action, score in confidence_scores.items():
        print(f"{action}: {score:.2f}")

def main():
    file_path = 'cursor_positions.csv'
    df, le = load_data(file_path)
    X, y = create_features(df)
    model = train_model(X, y)
    
    while True:
        predict_user_action(model, le)
        if input("Try again? (y/n): ").lower() != 'y':
            break

if __name__ == "__main__":
    main()