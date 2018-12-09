import React, { Component } from 'react';
import './App.css';
import 'c3/c3.css';
import LandtagView from './views/LandtagView'

class App extends Component {
  render() {
    return (
      <div className="App">
        <LandtagView/>
      </div>
    );
  }
}

export default App;
