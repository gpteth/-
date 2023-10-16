import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class AuditService {
  private apiUrl = '/api/audits';

  constructor(private http: HttpClient) {}

  getAudits(): Observable<any[]> {
    return this.http.get<any[]>(this.apiUrl);
  }

  createAudit(audit: any): Observable<any> {
    return this.http.post<any>(this.apiUrl, audit);
  }
}
