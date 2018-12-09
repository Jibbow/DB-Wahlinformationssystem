import React, { Component } from 'react';
import { Tabs, Tab } from 'react-bootstrap';
import './App.css';
import LandtagView from './views/LandtagView';
import WahlkreisView from './views/WahlkreisView';
import StimmkreisView from './views/StimmkreisView';

class App extends Component {
  render() {
    return (
      <div className="App">
        <Tabs defaultActiveKey={1} id="uncontrolled-tab-example">
          <Tab eventKey={1} title="Überblick Landtagswahl">
            <LandtagView/>
          </Tab>
          <Tab eventKey={2} title="Überblick Wahlkreise">
            <WahlkreisView/>
          </Tab>
          <Tab eventKey={3} title="Überblick Stimmkreise">
            <StimmkreisView/>
          </Tab>
        </Tabs>
      </div>
    );
  }
}

export default App;
