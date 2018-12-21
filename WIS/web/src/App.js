import React, { Component } from 'react';
import { Tabs, Tab } from 'react-bootstrap';
import './App.css';
import LandtagView from './views/LandtagView';
import WahlkreisView from './views/WahlkreisView';
import StimmkreisView from './views/StimmkreisView';
import ParteiView from './views/ParteiView';
import AnalysisView from './views/AnalysisView';
import { VoteButton } from './views/VoteView';

class App extends Component {
  render() {
    return (
      <div className="app">
        <VoteButton className="vote-button"/>

        <Tabs mountOnEnter={true} defaultActiveKey={1} id="uncontrolled-tab-example">
          <Tab eventKey={1} title="Überblick Landtagswahl">
            <LandtagView/>
          </Tab>
          <Tab eventKey={2} title="Überblick Wahlkreise">
            <WahlkreisView/>
          </Tab>
          <Tab eventKey={3} title="Überblick Stimmkreise">
            <StimmkreisView/>
          </Tab>
          <Tab eventKey={4} title="Parteien">
            <ParteiView/>
          </Tab>
          <Tab eventKey={5} title="Analysen">
            <AnalysisView/>
          </Tab>
        </Tabs>
      </div>
    );
  }
}

export default App;
