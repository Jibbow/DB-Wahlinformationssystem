import React, { Component } from 'react';
import { Button, Modal, FormGroup, ControlLabel, FormControl, Radio, Table } from 'react-bootstrap';

export class VoteButton extends Component {
  constructor(props, context) {
    super(props, context);

    this.handleIdentity = this.handleIdentity.bind(this);
    this.handleErststimme = this.handleErststimme.bind(this);
    this.handleErststimmeAbgeben = this.handleErststimmeAbgeben.bind(this);
    this.handleErststimmeEnthalten = this.handleErststimmeEnthalten.bind(this);
    this.handleZweitstimme = this.handleZweitstimme.bind(this);
    this.handleZweitstimmeAbgeben = this.handleZweitstimmeAbgeben.bind(this);
    this.handleZweitstimmeEnthalten = this.handleZweitstimmeEnthalten.bind(this);
    this.handleAbschluss = this.handleAbschluss.bind(this);
    this.handleClose = this.handleClose.bind(this);
    this.validateWahltokenState = this.validateWahltokenState.bind(this);

    this.state = {
      identity: false,
      erststimme: false,
      zweitstimme: false,
      abschluss: false,
      wahltoken: '',
      zweitstimmekandidaten: [], 
      erststimmekandidaten: [],
      jahr: 0,
      stimmkreis: 0,
      erststimmeabgegeben: 0,
      zweitstimmeabgegeben: 0,
      erststimmewahl: 0,
      zweitstimmewahlpartei: 0,
      zweitstimmewahlkandidat: 0
    };
  }

 /*updateWahltoken() {
    return fetch('http://localhost:8000/tokeninfo', {
      method: 'DELETE',
      body: JSON.stringify({id: this.state.wahltoken})
    })
    .then(
      fetch('http://localhost:8000/tokeninfo', {
        method: 'POST',
        body: formData
    }))
    .then(response => response.json())
  }*/
  

  validateWahltokenState() {
    fetch(`http://localhost:8000/tokeninfo/`.concat(this.state.wahltoken)) 
    .then(response => response.json())
    .then(data => {
      if (data.length == 1) {
          let tokeninfo = data[0];
          this.setState({stimmkreis: tokeninfo.STIMMKREIS});
          this.setState({erststimmeabgegeben: tokeninfo.ERSTSTIMMEABGEGEBEN});
          this.setState({zweitstimmeabgegeben: tokeninfo.ZWEITSTIMMEABGEGEBEN});
          this.setState({zweitstimmeabgegeben: 1});
          fetch(`http://localhost:8000/wahlzettel/erststimme/${this.state.stimmkreis}/${this.state.jahr}`)
          .then(response => response.json())
          .then(data => {
            this.setState({ erststimmekandidaten: data});
          });
          fetch(`http://localhost:8000/wahlzettel/zweitstimme/${this.state.stimmkreis}/${this.state.jahr}`)
          .then(response => response.json())
          .then(data => {
            let parteikandidaten_temp = this.getParteikandidaten(data);
            this.setState({ zweitstimmekandidaten: parteikandidaten_temp});
          });
          if (this.state.erststimmeabgegeben == false) {
            this.handleErststimme();
          }
          else if (this.state.zweitstimmeabgegeben == false) {
            window.error("Sie haben schon eine Erststimme abgegeben. Weiter zur Zweitstimme.");
            this.handleZweitstimme();
          }
          else {
            window.error("Sie haben schon eine Erststimme und eine Zweitstimme abgegeben.");
          }
      }
      else {
        window.alert("Ihr Wahltoken ist ungültig.");
      }
    })
    .catch(error => {
      window.alert("Ihr Wahltoken ist ungültig.");
    });

  }

  validateWahltoken() {
    if (this.state.wahltoken.match('^[0-9a-z]{8}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{12}$')) {
      return 'success';
    }
    return 'error';
  }

  getParteikandidaten(data) {
    let parteikandidaten_temp = [];
    data.map(p => {
      let gefunden = false;
      for (const pk of parteikandidaten_temp) {
        if (pk.partei.abkuerzung === p.PARTEI_ABKUERZUNG) {
          pk.kandidaten.push({position: p.LISTENPOSITION, name: p.KANDIDAT_NACHNAME, vorname: p.KANDIDAT_VORNAME});
          gefunden = true
        }
      }
      if (!gefunden) {
        parteikandidaten_temp.push({partei: { name: p.PARTEI, abkuerzung: p.PARTEI_ABKUERZUNG }, kandidaten: [{position: p.LISTENPOSITION, name: p.KANDIDAT_NACHNAME, vorname: p.KANDIDAT_VORNAME}]});
      }
    });
    return parteikandidaten_temp;
  }

  getPositionskandidaten() {
    let maxLength = 0;
    for (const pk of this.state.zweitstimmekandidaten) {
      if (pk.kandidaten.length > maxLength) {
        maxLength = pk.kandidaten.length;
      }
    }

    let positionskandidaten = [];
    for (let k = 0; k < maxLength; k++) {
      positionskandidaten.push([]);
    }

    this.state.zweitstimmekandidaten.map((pk, j) => {
      for (let i = 0; i < maxLength; i++) {
        if (pk.kandidaten.length > i){
          positionskandidaten[i].push(pk.kandidaten[i]);
        }
      }
    });
    return positionskandidaten;
  }

  render() {
    return (
      <div>
        <Button className="vote-button" bsSize="large" bsStyle="primary" onClick={this.handleIdentity}>
          Meine Stimme abgeben
        </Button>

        <Modal show={this.state.identity} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>Meine Stimme abgeben</Modal.Title>
          </Modal.Header>
          <Modal.Body>
              <FormGroup controlId="formWahltoken" validationState={this.validateWahltoken()}>
                <ControlLabel>Geben Sie bitte hier Ihren Wahltoken ein:</ControlLabel>
                <FormControl type="text" value={this.state.wahltoken} placeholder="z.B. 00000000-0000-0000-0000-000000000000" onChange={e => this.setState({ wahltoken: e.target.value })} />
                <FormControl.Feedback />
              </FormGroup>
          </Modal.Body>
          <Modal.Footer>
            <Button onClick={this.validateWahltokenState}>Wahl durchführen</Button>
          </Modal.Footer>
        </Modal>

        <Modal show={this.state.erststimme} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>Sie haben eine Stimme</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            <FormGroup>
            <div className = "stimmabgabeTable">
              <Table>
                <thead>
                  <tr>
                    {this.state.erststimmekandidaten.map((k, i) =>
                      <th>
                        <div>Wahlvorschlag Nr. {i + 1}</div>
                        <div>{k.PARTEI}</div>
                        <div>{k.PARTEI_ABKUERZUNG}</div>
                      </th>
                    )}
                  </tr>
                </thead>
                <tbody>
                  <tr>
                  {this.state.erststimmekandidaten.map(k =>
                    <td><Radio name="radioGroup" inline>
                      <div>{k.LISTENPOSITION}</div>
                      <div>{k.KANDIDAT_NACHNAME}</div>
                      <div>{k.KANDIDAT_VORNAME}</div>
                    </Radio></td>
                  )}
                  </tr>
                </tbody>
              </Table>
            </div>
            </FormGroup>
          </Modal.Body>
          <Modal.Footer>
          <Button onClick={this.handleErststimmeEnthalten}>Erststimme enthalten</Button>
            <Button onClick={this.handleErststimmeAbgeben}>Erststimme abgeben</Button>
          </Modal.Footer>
        </Modal>

        <Modal show={this.state.zweitstimme} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>Sie haben eine Stimme</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            <FormGroup>
              <div className="stimmabgabeTable">
                <Table striped bordered condensed hover>
                  <thead>
                    <tr>
                      {this.state.zweitstimmekandidaten.map((p, i) =>
                        <th><Radio name="radioGroup" inline>
                          <div>Wahlvorschlag Nr. {i + 1}</div>
                          <div>{p.partei.name}</div>
                          <div>{p.partei.abkuerzung}</div>
                        </Radio></th>
                      )}
                    </tr>
                  </thead>
                  <tbody>
                    {this.getPositionskandidaten().map((p, i) =>
                      <tr>
                        {p.map((k, j) =>
                          <td><Radio name="radioGroup" inline>
                            <div>{k.position} {k.name}, {k.vorname}</div>
                          </Radio></td>
                        )}
                      </tr>
                    )}
                  </tbody>
                </Table>
              </div>
            </FormGroup>
          </Modal.Body>
          <Modal.Footer>
            <Button onClick={this.handleZweitstimmeEnthalten}>Zweitstimme enthalten</Button>
            <Button onClick={this.handleZweitstimmeAbgeben}>Zweitstimme abgeben</Button>
          </Modal.Footer>
        </Modal>

        <Modal show={this.state.abschluss} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>
              <div>Vielen Dank.</div>
              <div>Ihre Stimme wurde gespeichert.</div>
            </Modal.Title>
          </Modal.Header>
        </Modal>

      </div>
    );
  }

  handleClose() {
    this.setState({ identity: false });
    this.setState({ erststimme: false });
    this.setState({ zweitstimme: false });
    this.setState({ abschluss: false });
  }

  handleIdentity() {
    this.setState({ identity: true });
    this.setState({ erststimme: false });
    this.setState({ zweitstimme: false });
    this.setState({ abschluss: false });
  }

  handleErststimme() {
    this.setState({ identity: false });
    this.setState({ erststimme: true });
    this.setState({ zweitstimme: false });
    this.setState({ abschluss: false });
  }

  handleErststimmeAbgeben() {
    this.setState({ erststimmeabgegeben: 1 });
    if (this.state.zweitstimmeabgegeben === 0) {
      this.handleZweitstimme();
    }
    else {
      this.handleAbschluss();
    }
  }

  handleErststimmeEnthalten() {
    this.setState({ erststimmeabgegeben: 1 });
    if (this.state.zweitstimmeabgegeben === 0) {
      this.handleZweitstimme();
    }
    else {
      this.handleAbschluss();
    }
  }

  handleZweitstimme() {
    this.setState({ identity: false });
    this.setState({ erststimme: false });
    this.setState({ zweitstimme: true });
    this.setState({ abschluss: false });
  }

  handleZweitstimmeAbgeben() {
    this.setState({ zweitstimmeabgegeben: 1 });
    this.updateWahltoken();
    this.handleAbschluss();
  }

  handleZweitstimmeEnthalten() {
    this.setState({ zweitstimmeabgegeben: 1 });
    this.updateWahltoken();
    this.handleAbschluss();
  }

  handleAbschluss() {
    this.setState({ identity: false });
    this.setState({ erststimme: false });
    this.setState({ zweitstimme: false });
    this.setState({ abschluss: true });
  }
}
