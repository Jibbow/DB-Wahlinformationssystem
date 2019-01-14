import React, { Component } from 'react';
import { Navbar, Nav, NavItem, Tab, Checkbox, FormGroup } from 'react-bootstrap';
import './App.css';
import LandtagView from './views/LandtagView';
import WahlkreisView from './views/WahlkreisView';
import StimmkreisView from './views/StimmkreisView';
import ParteiView from './views/ParteiView';
import AnalysisView from './views/AnalysisView';
import { VoteButton } from './views/VoteView';
import Toggle from 'react-toggle';
import "react-toggle/style.css";

class App extends Component {
  constructor(props, context) {
    super(props, context);

    this.state = {
      useAggregatedData: false,
    };
  }


  render() {
    return (
      <Tab.Container className="app" defaultActiveKey={1}>
        <div>
          <Navbar fixedTop fluid className="header">
            <Navbar.Header>
              <Navbar.Toggle />
            </Navbar.Header>
            <Navbar.Collapse>
              <Nav>
                <NavItem eventKey={1}>Landtagswahl</NavItem>
                <NavItem eventKey={2}>Wahlkreise</NavItem>
                <NavItem eventKey={3}>Stimmkreise</NavItem>
                <NavItem eventKey={4}>Parteien</NavItem>
                <NavItem eventKey={5}>Analysen</NavItem>
              </Nav>
              <Navbar.Form pullRight>
                <FormGroup>
                  <label className="checkbox-aggregate">
                    <span>Auf aggregierten Daten arbeiten</span>
                    <Toggle checked={this.state.useAggregatedData} icons={false} onChange={e => this.setState({useAggregatedData: e.target.checked})}/>
                  </label>
                </FormGroup>
                <FormGroup>
                  <VoteButton />
                </FormGroup>
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
    );
  }
}

export default App;
