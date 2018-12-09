import React, { Component } from 'react';
import { Tabs, Tab } from 'react-bootstrap';
import './App.css';
import 'c3/c3.css';
import LandtagView from './views/LandtagView'

class App extends Component {
  render() {
    return (
      <div className="App">
        <Tabs defaultActiveKey={2} id="uncontrolled-tab-example">
          <Tab eventKey={1} title="Überblick Landtagswahl">
            <LandtagView/>
          </Tab>
          <Tab eventKey={2} title="Tab 2">
            Tab 2 content
          </Tab>
          <Tab eventKey={3} title="Tab 3">
            Tab 3 content
          </Tab>
        </Tabs>;
      </div>
    );
  }
}

export default App;
