import React, { Component } from 'react';
import { Navbar, Nav, NavItem, Tab } from 'react-bootstrap';
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
        <Tab.Container defaultActiveKey={1}>
          <div>
            <Navbar fixedTop fluid className="header">
              <Navbar.Header>
                <Navbar.Toggle />
              </Navbar.Header>
              <Navbar.Collapse>
                <Nav>
                  <NavItem eventKey={1}>Überblick Landtagswahl</NavItem>
                  <NavItem eventKey={2}>Überblick Wahlkreise</NavItem>
                  <NavItem eventKey={3}>Überblick Stimmkreise</NavItem>
                  <NavItem eventKey={4}>Parteien</NavItem>
                  <NavItem eventKey={5}>Analysen</NavItem>
                </Nav>
                <Navbar.Form pullRight>
                  <VoteButton />
                </Navbar.Form>
              </Navbar.Collapse>
            </Navbar>

            <Tab.Content mountOnEnter={true}>
              <Tab.Pane eventKey={1}>
                <LandtagView />
              </Tab.Pane>
              <Tab.Pane eventKey={2}>
                <WahlkreisView />
              </Tab.Pane>
              <Tab.Pane eventKey={3}>
                <StimmkreisView />
              </Tab.Pane>
              <Tab.Pane eventKey={4}>
                <ParteiView />
              </Tab.Pane>
              <Tab.Pane eventKey={5}>
                <AnalysisView />
              </Tab.Pane>
            </Tab.Content>
          </div>
        </Tab.Container>
      </div>
    );
  }
}

export default App;
