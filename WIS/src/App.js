import React, { Component } from 'react';
import './App.css';

class App extends Component {
  constructor() {
    super();
    this.state = {
      result: 0,
    };
    this.getResult = this.getResult.bind(this);
  }

  render() {
    return (
      <div className="App">
        <header className="App-header">
          <p>{this.state.result}</p>
          <button onClick={this.getResult}>click!</button>
        </header>
      </div>
    );
  }

  getResult() {
    this.setState({
      result: 'yeah!',
    });
  }
}

export default App;
