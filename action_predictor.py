import pandas as pd
import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import LabelEncoder
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import accuracy_score
import joblib

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

def main():
    file_path = 'cursor_positions.csv'
    df, le = load_data(file_path)
    X, y = create_features(df)
    model = train_model(X, y)
    
    # Save the model and label encoder to disk
    joblib.dump(model, 'action_model.pkl')
    joblib.dump(le, 'label_encoder.pkl')
    print("Model and label encoder saved to disk.")

if __name__ == "__main__":
    main()