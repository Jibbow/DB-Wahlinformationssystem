import React, { Component } from 'react';
import { Button, Modal, FormGroup, ControlLabel, FormControl, HelpBlock, Popover, OverlayTrigger } from 'react-bootstrap';

export class VoteButton extends Component {
  constructor(props, context) {
    super(props, context);

    this.handleShow = this.handleShow.bind(this);
    this.handleClose = this.handleClose.bind(this);

    this.state = {
      show: false,
      ausweisnummer: '',
    };
  }

  validateAusweisnummer() {
    const length = this.state.ausweisnummer.length;
    if (length === 9) return 'success';
    else if (length > 0) return 'warning';
    return null;
  }

  render() {
    return (
      <div>
        <Button className="vote-button" bsSize="large" bsStyle="primary" onClick={this.handleShow}>
          Meine Stimme abgeben
        </Button>

        <Modal show={this.state.show} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>Meine Stimme abgeben</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            <OverlayTrigger
              placement="bottom"
              trigger={['focus']}
              overlay={
                <Popover id="popover-trigger-hover-focus" title="Personalausweisnummer">
                  <img id="personalausweis" src={require('../assets/personalausweis.jpg')} alt="Personalausweisnummer" />
                </Popover>
              }>
              <FormGroup controlId="formPersonalausweisnummer" validationState={this.validateAusweisnummer()}>
                <ControlLabel>Geben Sie bitte hier ihre Personalausweisnummer ein:</ControlLabel>
                <FormControl type="text" value={this.state.ausweisnummer} placeholder="z.B. T22000129" onChange={e => this.setState({ ausweisnummer: e.target.value })} />
                <FormControl.Feedback />
                <HelpBlock>
                  <p>Die Personalausweisnummer befindet sich oben rechts auf Ihrem Ausweisdokument</p>
                </HelpBlock>
              </FormGroup>
            </OverlayTrigger>
          </Modal.Body>
          <Modal.Footer>
            <Button onClick={this.handleClose}>Wahl durchführen</Button>
          </Modal.Footer>
        </Modal>
      </div>
    );
  }

  handleClose() {
    this.setState({ show: false });
  }

  handleShow() {
    this.setState({ show: true });
  }
}
