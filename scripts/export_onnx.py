#!/usr/bin/env python3
"""
Script to export simple ONNX models for testing
"""

import torch
import torch.nn as nn
import torch.onnx

class SimpleModel(nn.Module):
    def __init__(self):
        super().__init__()
        self.linear = nn.Linear(10, 5)
        self.relu = nn.ReLU()
        
    def forward(self, x):
        x = self.linear(x)
        x = self.relu(x)
        return x

def main():
    # Create model
    model = SimpleModel()
    model.eval()
    
    # Create dummy input
    dummy_input = torch.randn(1, 10)
    
    # Export to ONNX
    torch.onnx.export(
        model,
        dummy_input,
        "models/example.onnx",
        export_params=True,
        opset_version=11,
        do_constant_folding=True,
        input_names=['input'],
        output_names=['output'],
        dynamic_axes={
            'input': {0: 'batch_size'},
            'output': {0: 'batch_size'}
        }
    )
    
    print("✅ Exported model to models/example.onnx")

if __name__ == "__main__":
    main()
